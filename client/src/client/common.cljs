(ns client.common
  (:require [cljs-react-material-ui.reagent :as ui]
            [ajax.core :refer [GET POST]]))

(defn paper-rounded [child]
  [ui/paper {:style {:margin 20
                     :padding 20
                     :rounded true
                     :border-radius 5
                     :text-align "center"}}
   child])

(defn v-or-blank [state ks]
  (if-let [v (get-in @state ks)] v ""))
