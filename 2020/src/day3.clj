(ns day3
  (:require
   [util]))

(defn parse [input]
  (->> input util/input-list (into [])))

(defn count-trees [right down lines]
  (letfn [(tree? [row]
            (let [line (get lines row)]
              (= \# (get line (mod (* right (/ row down)) (count line))))))]
    (count (filter tree? (range 0 (count lines) down)))))

(defn p1 [input]
  (->> input parse (count-trees 3 1)))

(defn p2 [input]
  (let [lines (parse input)]
    (apply * (map (fn [[right down]] (count-trees right down lines))
                  [[1 1] [3 1] [5 1] [7 1] [1 2]]))))

(defn -main []
  (util/download-input 3)
  (let [input (util/read-input 3)]
    (println (p1 input))
    (println (p2 input))
    (shutdown-agents)))
