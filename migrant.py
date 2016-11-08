#!/usr/bin/python3

import os
import sys
import subprocess

from collections import namedtuple
from datetime import datetime

HELP = """***************************
<>  Migrant <>
     --help: show this message
     --list: (default) Show applied migrations and all available.
     --up: Move forward one migration.
     --down: Move back one migration.
     --new [name]: Generate a new blank migration with an optional
                   name (defaults to unnamed).
     --merge: Merges diverging histories. Rolls back applied migrations
              to the last aligned migration, then fast-forwards to the
              migration that this command was run at.
     --reset: Clears migration history. Does not actually run any
              of the 'down' migrations.
"""
# TODO:
#     [id]: `1.unnamed` move to the migration, applying any
#            up's or down's to get there.
#"""

THIS_DIR = os.path.dirname(os.path.abspath(__file__))
PROJ_NAME = os.path.basename(THIS_DIR)
MIGRANT_META = os.path.join(THIS_DIR, '.migrant')
MIGRATION_DIR = os.path.join(THIS_DIR, 'resources', 'migrations')
DT_FORMAT = '%Y%m%d-%H%M%S'


def load_or_prompt():
    try:
        f = open(MIGRANT_META)
        meta = [line.rstrip('\n') for line in f]
        f.close()
    except FileNotFoundError:
        print(' > No .migrant meta file found for this project.\n'
              ' > Would you like to initialize migrant?')
        ans = input('    y/n >> ').strip().lower()
        if ans != 'y':
            return
        with open(MIGRANT_META, 'w') as f:
            f.write(PROJ_NAME)
        print("** Initializing...")
        meta = [PROJ_NAME]
    return meta


def shorten(path):
    fname = os.path.basename(path)
    folder = os.path.basename(os.path.dirname(path))
    return os.path.join(folder, fname)


def show(meta, available):
    print('- - - - - - - - - - - - - - - - - - - - - - - - - - ')
    print('\n** Migration info for project: {} **'.format(meta[0]))

    applied = meta[1:]
    print('\n--> Applied:')
    if not applied:
        print('   ** No migrations applied')
    else:
        for line in applied:
            print('  ->> {}'.format(shorten(line)))

    print('\n--> Migration files found:')
    if not available:
        print('   ** No available migrations found')
    else:
        for up in available:
            print('  --> {name} <-> [{x}]'.format(
                name=shorten(up.name),
                x='X' if up.up_path in applied else ' '))


def search_for_migrations():
    available = []
    MigFile = namedtuple('MigFile', ['name', 'up_path', 'down_path', 'm_id', 'dt'])
    if not os.path.exists(MIGRATION_DIR):
        print('\n ** `{}` directory not found!'.format(shorten(MIGRATION_DIR)))
        print(' ** Would you like to create it now?')
        ans = input('    y/n >> ').strip().lower()
        if ans == 'y':
            os.makedirs(MIGRATION_DIR)
            return search_for_migrations()
    else:
        for root, dirs, files in os.walk(MIGRATION_DIR):
            for name in files:
                if name.startswith('up.') and name.endswith('.sql'):
                    mid = name.split('.')[1:-1]
                    m_id, dt = mid[0], mid[-1]
                    mf = MigFile(name=name, up_path=os.path.join(root, name),
                                 down_path=os.path.join(root, name.replace('up.', 'down.')),
                                 m_id=m_id, dt=datetime.strptime(dt, DT_FORMAT))
                    available.append(mf)
        available.sort(key=lambda mf: mf.dt)
    return available


def run_migration(migration_file, up_down):
    command = subprocess.run(['sudo', '-u', PROJ_NAME, 'psql', '-U',
                    PROJ_NAME, '-d', PROJ_NAME, '-f',
                    migration_file.up_path if up_down == 'up'
                    else migration_file.down_path], check=True,
                    stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    print('{}'.format(command.stdout.decode('utf-8')))
    if command.stderr:
        print('\n ** Woops, something\'s not right...')
        print(command.stderr.decode('utf-8'))
        return False
    return True
    #os.system('sudo -u {projuser} psql -U {pguser} -d {dbname} -f {path}'.format(
    #        projuser=PROJ_NAME,
    #        pguser=PROJ_NAME,
    #        dbname=PROJ_NAME,
    #        path=migration_file.up_path if up_down == 'up' else migration_file.down_path,
    #    ))


def update_meta(meta, mig, up_down):
    index = 1
    found = False
    for line in meta[1:]:
        if line == mig.up_path:
            found = True
            break
        index += 1
    if found:
        meta = meta[:index]
    else:
        if up_down == 'up':
            # safe to assume the migration was run
            meta.append(mig.up_path)
        else:
            # not safe to assume migration has been run.
            # this migration is missing from applied history
            # so it was probably created by someone else.
            # just chop off the current applied-head.
            meta = meta[:-1]
    with open(MIGRANT_META, 'w') as f:
        f.write('\n'.join(meta))
    return meta


def clear_meta(meta):
    meta = meta[:1]
    with open(MIGRANT_META, 'w') as f:
        f.write('\n'.join(meta))
    return meta


def apply_next(direction, meta, available):
    if not available:
        print('No migration files available')
        return meta

    applied = meta[1:]
    if not applied:
        up_next_index = 0
        down_next_index = None
    else:
        current_mig_path = applied[-1]
        cur_index = 0
        for mf in available:
            if mf.up_path == current_mig_path:
                break
            cur_index += 1
        up_next_index = cur_index + 1
        down_next_index = cur_index

    if direction == 'up':
        if up_next_index < len(available):
            # call it
            mig = available[up_next_index]
            check = run_migration(mig, 'up')
            if check:
                meta = update_meta(meta, mig, 'up')
                print('** Moved up to:\n{}'.format(shorten(meta[-1])))
        else:
            print('**** Already at latest migration ****')
    else:
        if down_next_index is not None:
            mig = available[down_next_index]
            check = run_migration(mig, 'down')
            if check:
                meta = update_meta(meta, mig, 'down')
                print('** Moved down to:\n{}'.format(shorten(meta[-1])))
        else:
            print('**** No migrations to move down from ****')
    return meta


def make_new_migration(name, n):
    dt = datetime.now().strftime(DT_FORMAT)
    dir_name = '{n}.{name}.{dt}'.format(n=n, name=name, dt=dt)
    up_file = 'up.{n}.{name}.{dt}.sql'.format(n=n, name=name, dt=dt)
    down_file = 'down.{n}.{name}.{dt}.sql'.format(n=n, name=name, dt=dt)
    new_dir = os.path.join(MIGRATION_DIR, dir_name)
    os.makedirs(new_dir)
    for fname in [up_file, down_file]:
        with open(os.path.join(new_dir, fname), 'w') as f:
            f.write('')
    print('** Generated new up & down templates:\n'
          '<> {path}\n'
          '  -> {up}\n'
          '  -> {down}'.format(path=new_dir, up=up_file, down=down_file))


def find_available_index(path, available):
    for i, mig in enumerate(available):
        if path == mig.up_path:
            return i
    return


def find_applied_index(path, applied):
    if not applied:
        return
    for i, mig_path in enumerate(applied):
        if mig_path == path:
            return i
    return


def last_aligned_migration(applied, available):
    len_app = len(applied)
    if len_app == 0: return True, 0
    len_ava = len(available)
    n = min(len_app, len_ava)
    for i in range(n):
        if applied[i] != available[i].up_path:
            return False, i
    if len_ava >= len_app:
        return True, i


def rollback_to(roll_to_index, meta, available):
    applied = meta[1:]
    applied = [(i, a) for i, a in enumerate(applied)]
    applied.reverse()
    clear_all = False
    if roll_to_index < 0:
        clear_all = True
    for i, path in applied:
        if not clear_all:
            if i <= roll_to_index:
                break
        avail_index = find_available_index(path, available)
        if avail_index:
            mig = available[avail_index]
            check = run_migration(mig, 'down')
            if not check:
                return
            meta = update_meta(meta, mig, 'down')
            print('** Moved down to:\n{}'.format(shorten(meta[-1])))
    return meta


def replay_to(path, meta, available):
    applied_index = find_applied_index(path, meta[1:])
    if applied_index:
        # already applied
        return
    head = meta[-1]
    while head != path:
        meta = apply_next('up', meta, available)
        head = meta[-1]


def find_available_index_by_partial(partial_path, available):
    for i, mig in enumerate(available):
        if mig.up_path.startswith(mig.up_path) or mig.down_path.startswith(mig.down_path):
            return i

def force_single(mig_id, up_down, meta, available):
    index = find_available_index_by_partial(mig_id, available)
    if index is not None:
        mig = available[index]
        check = run_migration(mig, up_down)
        if check:
            meta = update_meta(meta, mig, 'up')
            print('** Moved up to:\n{}'.format(shorten(meta[-1])))
    return meta


def run(args, meta):
    available = search_for_migrations()
    if not args:
        show(meta, available)
        return
    arg = args[0]
    if arg == '--help':
        print(HELP)
    elif arg in ['--up', '--down', '-u', '-d']:
        _dir = {'--up': 'up', '--down': 'down', '-u': 'up', '-d': 'down'}[arg]
        meta = apply_next(_dir, meta, available)
        show(meta, available)
    elif arg == '--new':
        name = 'unnamed'
        if len(args) >= 2:
            name = args[1].lower()
        make_new_migration(name, len(available))
        available = search_for_migrations()
        show(meta, available)
    elif arg == '--merge':
        ok, index = last_aligned_migration(meta[1:], available)
        if not ok:
            head = meta[-1]
            print("\n*** Rolling back:")
            meta = rollback_to(index-1, meta, available)
            if meta:
                print("\n *** Replaying:")
                replay_to(head, meta, available)
        show(meta, available)
    elif arg == '--reset':
        print('\n ** This will clear your personal migration history')
        print(' ** saved in: {}'.format(shorten(MIGRANT_META)))
        ans = input(' ** Are you sure you want to do this?\n   y/n >> ').strip().lower()
        if ans == 'y':
            meta = clear_meta(meta)
            show(meta, available)
    elif arg == '--run':
        try:
            up_down, n, name = args[1].split('.')
            _ = int(n)
        except ValueError:
            print(' ** Make sure input follows: `--run up.1.migname` ')
            return
        meta = force_single(arg.lower(), up_down, meta, available)
        show(meta, available)


if __name__ == '__main__':
    meta = load_or_prompt()
    if meta is not None:
        run(sys.argv[1:], meta)
