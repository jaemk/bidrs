(ns client.navigation
  (:require [cljs-react-material-ui.reagent :as ui]))

(defn toggle-nav [state]
  (swap! state update-in [:nav :open] not))

(defn nav [state]
  [:div
   [ui/app-bar {:title (get-in @state [:nav :title])
                :on-left-icon-button-touch-tap #(toggle-nav state)
                :title-style {:margin-right "48px"}
                :z-depth 0
                :show-menu-icon-button true}]
   [ui/drawer {:docked false
               :open (get-in @state [:nav :open])
               :on-request-change #(toggle-nav state)}]])

