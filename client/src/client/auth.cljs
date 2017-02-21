(ns client.auth
  (:require [cljs-react-material-ui.reagent :as ui]
            [reagent.core :as r]
            [secretary.core :refer [dispatch!]]
            [client.common :refer [v-or-blank]]))


(defn check-params [local]
  (println "checking params...")
  (let [email (:email @local)
        pass  (:password @local)
        params [email pass]]
    (if (every? #(not (nil? %)) params)
      {:email email :password pass}
      (do
        (println "err")
        (doseq [[k v] (map vector [:email :password] params)]
          (when (nil? v)
            (swap! local assoc-in [:errors v] true)))
        false))))


(defn login-handle [global resp]
  (let [set-token! (get-in @global [:api :set-token!])]
    (set-token! (:token resp))
    (swap! global assoc-in [:user :authenticated] true)
    ((-> @global :nav :goto) :home)
    ))



(defn submit [global local]
  (when-let [post-params (check-params local)]
    ((-> @global :api :post)
      :url "/login"
      :data post-params
      :handler #(login-handle global %))))


(defn login [global]
  (let [local (r/atom {:email nil
                       :password nil
                       :errors {}})]
    (fn []
      [:div
       [ui/text-field {:floating-label-text "email"
                       :value (v-or-blank local [:email])
                       :error-text (if (not (nil? (get-in @local [:errors :email]))) "Required")
                       :on-change #(swap! local assoc-in [:email] (-> % .-target .-value))}]
       [:br]
       [ui/text-field {:floating-label-text "password"
                       :value (v-or-blank local [:password])
                       :on-change #(swap! local assoc-in [:password] (-> % .-target .-value))
                       :type "password"}]
       [:br]
       [ui/raised-button {:label "submit"
                          :on-touch-tap #(submit global local)}]])))
