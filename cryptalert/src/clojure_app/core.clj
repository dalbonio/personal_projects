(ns clojure-app.core
  (:require [clojure.data.json :as json] [clj-http.client :as client] [postal.core :as postal]))


(def crypto-list (list '("BTC", 65000.00), '("ETH", 1650.00), '("CHZ", 0.9), '("DOGE", 0.04), '("ADA", 1.05), '("LTC", 190.00), '("VET", 0.07), '("DOT", 30.00), '("XRP", 0.35), '("XLM", 0.35)))

(def email-list '("dalbonio2@gmail.com"))

(defn -main
  "I don't do a whole lot ... yet."
  [& args]
  (println "Hello, World!"))

(defn train
  []
  "Choo choo!")

(defn map-api-data
  [api-data]
           (map (fn [each_coin_data]
                  (get-in each_coin_data ["quote", "USD", "price"]))
                (map (fn [coin] (first (get-in (json/read-str api-data) ["data", coin]))) (map first crypto-list))))

(defn add-current-price
  [crypto-data]
  (map (fn [element]
         (conj (first element) (nth element 1)))
       (partition 2 (interleave crypto-list crypto-data))))

(defn get-crypto-data
  [api-data]
  (add-current-price (map-api-data api-data)))


(defn filter-crypto-data
  [crypto-data]
  (map (fn [el] (list (str "$" (first el)) (nth el 1))) (filter #(<= (first %) (nth % 2)) crypto-data)))

(defn curl-crypto
  []
  (client/get "https://pro-api.coinmarketcap.com/v2/cryptocurrency/quotes/latest" {:headers {"X-CMC_PRO_API_KEY" (System/getenv "COINMARKETAPIKEY")} :accept :json :query-params {"symbol" (clojure.string/join "," (map first crypto-list))}}))

(defn send-crypto-email
  [emailbody]
  (println (System/getenv "CLOJ_EMAIL"))
  (println (System/getenv "CLOJ_PASS"))
  (postal/send-message {:host "smtp.gmail.com"
                        :user (System/getenv "CLOJ_EMAIL")
                        :pass (System/getenv "CLOJ_PASS")
                        :port 587
                        :tls true}
                       {:from (System/getenv "CLOJ_EMAIL")
                        :to email-list
                        :subject "compra moeda ae"
                        :body emailbody}))

