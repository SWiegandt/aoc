(ns day5
  (:require
   [clojure.string :as str]
   [util]))

(defn parse [input]
  (-> input
      (str/replace #"[RB]" "1")
      (str/replace #"[LF]" "0")
      util/input-list))

(defn seat-id [row]
  (let [[row col] (split-at 7 row)]
    (-> (Long/parseLong (str/join col) 2)
        (+ (* 8 (Long/parseLong (str/join row) 2))))))

(defn p1 [input]
  (->> input parse (map seat-id) (apply max)))

(defn p2 [input]
  (let [seat-ids (->> input parse (map seat-id) (into #{}))]
    (->> seat-ids
         (filter #(and (not (contains? seat-ids (+ 1 %))) (contains? seat-ids (+ 2 %))))
         first
         inc)))

(defn -main []
  (util/download-input 5)
  (let [input (util/read-input 5)]
    (println (p1 input))
    (println (p2 input))
    (shutdown-agents)))
