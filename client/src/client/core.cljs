(ns client.core
  (:require-macros [secretary.core :refer [defroute]])
  (:import goog.History)
  (:require [reagent.core :as r]
            [cljsjs.material-ui]
            [cljs-react-material-ui.core :refer [get-mui-theme color]]
            [cljs-react-material-ui.reagent :as ui]
            [cljs-react-material-ui.icons :as ic]
            [ajax.core :refer [GET POST]]
            [secretary.core :as secretary]
            [goog.events :as events]
            [goog.history.EventType :as EventType]

            [client.navigation :as navigation]
            [client.common :as common :refer [v-or-blank]]
            [client.auth :as auth]
            [client.browse :as browse]))

(enable-console-print!)


;; global app state
(defonce state (r/atom
  {:user {:name nil
          :authenticated false}
   :api {:token nil
         :set-token! nil  ; fn
         :get nil         ; fn
         :post nil        ; fn
         }
   :nav {:title "Bidrs"
         :open false
         :page :home
         :goto nil        ; fn
         }
   :items nil
   :refresh {:browse nil  ; fn
             :home nil    ; fn
             }
   :toast {:msg nil}
   :msg nil}))


(defn set-api-token!
  "Set the current api token!"
  [token]
  (swap! state assoc-in [:api :token] token))


(defn default-err-handle
  "Default api ajax error handler, redirects to login page
  when unauthorized."
  [resp]
  (when (= 401 (:status resp))
    (set-api-token! nil)
    (swap! state assoc-in [:user :authenticated] false))
  (println resp))


(defn api-get
  "cljs-ajax/GET wrapped with our params"
  [& {:keys [url handler err-handler]
      :or   {handler #(println %)
             err-handler default-err-handle}}]
  (GET url {:headers {"Authorization" (get-in @state [:api :token])}
            :response-format :json
            :keywords? true
            :handler handler
            :error-handler err-handler}))


(defn api-post
  "cljs-ajax/POST wrapped with our params"
  [& {:keys [url data handler err-handler]
      :or   {handler #(println %)
             err-handle default-err-handle}}]
  (POST url {:headers {"Authorization" (get-in @state [:api :token])}
             :body (.stringify js/JSON (clj->js data))
             :response-format :json
             :keywords? true
             :handler handler
             :error-handler err-handler}))

(defn fetch-items []
  (api-get :url "/items"
           :handler (fn [resp]
                      (println resp)
                      (swap! state assoc :items (:items resp)))))

(defn page->title [page]
  (get {:home "Home"
        :browse "Browse"}
       page))

(defn page->uri [page]
  (get {:home "/home"
        :browse "/browse"}
       page))

(defn goto-page [page]
  (swap! state assoc-in [:nav :page] page)
  (secretary/dispatch! (page->uri page)))

;; shove functions into the global r/atom
(def forwards
  [[[:api :set-token!] set-api-token!]
   [[:api :get] api-get]
   [[:api :post] api-post]
   [[:nav :goto] goto-page]
   [[:refresh :browse] fetch-items]
   [[:refresh :home] #(println "refreshing home")]])
(doseq [[ks v] forwards]
  (swap! state assoc-in ks v))


(defn themer
  "generate our mui-theme"
  []
  (get-mui-theme
    {:palette
       {:text-color "#333333"}
     :app-bar
       {:color (color :amber600)
        :height 50}
     :tabs
       {:background-color (color :amber600)}}))


(defn test-get
  []
  [:div {:style {:font-weight "bold"}}
   [:div (v-or-blank state [:msg])]
   [ui/raised-button {:label "get!"
                      :on-touch-tap (fn [&_] (api-get
                                               :url "/hello"
                                               :handler #(swap! state assoc :msg (:data %))))
                      }]])

(defn test-whoami
  []
  [:div {:style {:font-weight "bold"}}
   [:div (v-or-blank state [:whoami])]
   [ui/raised-button {:label "whoami?"
                      :on-touch-tap (fn [&_] (api-get
                                               :url "/info"
                                               :handler #(swap! state assoc :whoami (:name %))
                                               ;:err-handler default-err-handle
                                               ))}]])

(defn browse [state]
  [common/paper-rounded
   [browse/items state]])

(defn home [state]
  "Main content"
  [common/paper-rounded
   [:div
     [test-get]
     [:br]
     [test-whoami]
     [:div "sup"]]])


(defn hook-browser-navigation! []
  (doto (History.)
    (events/listen
      EventType/NAVIGATE
      (fn [event]
        (secretary/dispatch! (.-token event))))
    (.setEnabled true)))

(defn app-routes []
  (secretary/set-config! :prefix "#")
  (defroute "/" []
    (swap! state assoc-in [:nav :page] :home))
  (defroute "/home" []
    (swap! state assoc-in [:nav :page] :home))
  (defroute "/browse" []
    (swap! state assoc-in [:nav :page] :browse))
  (hook-browser-navigation!))


(defmulti current-page #(get-in @state [:nav :page]))
(defmethod current-page :home [] [home state])
(defmethod current-page :browse [] [browse state])
(defmethod current-page :default [] [home state])


(defn content []
  (let [page (-> @state :nav :page)]
    [ui/tabs {:value page
              :on-change #((-> @state :nav :goto) (keyword %))
              :style {:width "100%"}
              }
     [ui/tab {:label "Home" :value :home}
      [home state]]
     [ui/tab {:label "Browse" :value :browse}
      [browse state]]]))

(defn main []
  [ui/mui-theme-provider {:mui-theme (themer)}
    [:div
     [navigation/nav state]
     (if (not (get-in @state [:user :authenticated]))
       [auth/login state]
       [content])]])
       ;[current-page])]])


(app-routes)
(r/render-component [main]
  (. js/document (getElementById "app")))

(defn on-js-reload []
  ;; optionally touch your app-state to force rerendering depending on
  ;; your application
  ;; (swap! app-state update-in [:__figwheel_counter] inc)
)
