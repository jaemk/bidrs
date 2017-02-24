(ns client.browse
  (:require [reagent.core :as r]
            [cljs-react-material-ui.reagent :as ui]))


(defn items [global]
  [:div
   [:div {:style {:font-weight "bold"}}
    (for [[i item] (map vector (range) (:items @global))]
      ^{:key i}
      [:div (:description item)])]])
