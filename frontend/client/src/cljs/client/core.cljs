(ns client.core
  (:require [reagent.core :as reagent :refer [atom]]
            [ajax.core :refer [GET POST]]))

(enable-console-print!)

(defonce config (atom {:token nil}))

(defonce state (atom {:text "Hello Chestnut!"
                      :user nil
                      :fields {}
                      }))

(defn api-get [url handler]
  (println (:token @config))
  (GET url {:handler handler
            :error-handler #(println %)
            :headers {"Authorization" (:token @config)}
            :response-format :json
            :keywords? true}))

(defn login [email password]
  (api-get "/login"
           (fn [resp]
             (let [token (:token resp)]
               (swap! config assoc-in [:token] token)))))

(defn current-user []
  (let [user (:user @state)]
    [:div (if (empty? user) "No User" user)]))

(defn text-input [key-name]
  [:input {:type "text"
           :value (get-in @state [:fields key-name])
           :on-change #(swap! state assoc-in [:fields key-name] (-> % .-target .-value))}])

(defn login-form []
  [text-input :email])

(defn home []
  [:div
   [:div (get-in @state [:fields :email] "No Message")]
   [current-user]
   [login-form]
   [:br]
   [:button {:on-click #(api-get "/hello" (fn [resp]
                                            (println resp)
                                            (swap! state assoc-in [:text] (:data resp))))}
    "click"]
   ])

(reagent/render [home] (js/document.getElementById "app"))
