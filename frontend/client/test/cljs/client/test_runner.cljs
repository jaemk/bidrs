(ns client.test-runner
  (:require
   [doo.runner :refer-macros [doo-tests]]
   [client.core-test]
   [client.common-test]))

(enable-console-print!)

(doo-tests 'client.core-test
           'client.common-test)
