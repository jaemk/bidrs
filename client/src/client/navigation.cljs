(ns client.navigation
  (:require [cljs-react-material-ui.reagent :as ui]
            [cljs-react-material-ui.icons :as ic]
            [reagent.core :as r]))

(defn toggle-nav [state]
  (swap! state update-in [:nav :open] not))

(defn refresh [state]
  (let [page (get-in @state [:nav :page])
        refresh-fn (get-in @state [:refresh page])]
    (println "refreshing...")
    (refresh-fn)))

(defn nav [state]
  [:div
   [ui/app-bar {:title (get-in @state [:nav :title])
                :on-left-icon-button-touch-tap #(toggle-nav state)
                :title-style {:margin-right "0px"}
                :z-depth 0
                :show-menu-icon-button true
                :icon-element-right (r/as-element [ui/icon-button (ic/navigation-refresh)])
                :on-right-icon-button-touch-tap #(refresh state)
                }]
   [ui/drawer {:docked false
               :open (get-in @state [:nav :open])
               :on-request-change #(toggle-nav state)}]])

