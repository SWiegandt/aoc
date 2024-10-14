(ns day2
  (:require
   [util]))

(defn parse-line [line]
  (let [[min' max' [ch] password] (rest (re-matches #"(\d+)-(\d+) (\w): (\w+)" line))]
    [(parse-long min') (parse-long max') ch password]))

(defn parse [input]
  (util/input-list input parse-line))

(defn valid-1? [line]
  (let [[min' max' ch password] line
        n (->> password (filter #(= ch %)) count)]
    (and (>= n min') (<= n max'))))

(defn valid-2? [line]
  (let [[min' max' ch password] line
        get-char (fn [pos] (get password (- pos 1)))
        min-char (get-char min')
        max-char (get-char max')]
    (not= (= ch min-char) (= ch max-char))))

(defn p1 [input]
  (->> input parse (filter valid-1?) count))

(defn p2 [input]
  (->> input parse (filter valid-2?) count))

(defn -main []
  (util/download-input 2)
  (let [input (util/read-input 2)]
    (println (p1 input))
    (println (p2 input))
    (shutdown-agents)))
