(ns client.browse
  (:require [reagent.core :as r]
            [cljs-react-material-ui.reagent :as ui]))

(def local (r/atom {:items nil}))

(defn fetch-items [global]
  (let [api-get (get-in @global [:api :get])]
    (api-get :url "/items"
             :handler (fn [resp]
                        (println resp)
                        (swap! local assoc :items (:items resp))))))

(defn items [global]
  (fetch-items global)
  (fn []
    [:div
     [:div {:style {:font-weight "bold"}}
      (for [[i item] (map vector (range) (:items @local))]
        ^{:key i}
        [:div (:description item)])]]))
