(ns day1
  (:require
   [util]))

(defn parse [input]
  (util/input-list input parse-long))

(defn find-sum-2 [xs]
  (for [x1 xs
        x2 xs
        :when (= (+ x1 x2) 2020)]
    (* x1 x2)))

(defn find-sum-3 [xs]
  (for [x1 xs
        x2 xs
        x3 xs
        :when (= (+ x1 x2 x3) 2020)]
    (* x1 x2 x3)))

(defn p1 [input]
  (-> input parse find-sum-2 first))

(defn p2 [input]
  (-> input parse find-sum-3 first))

(defn -main []
  (util/download-input 1)
  (let [input (util/read-input 1)]
    (println (p1 input))
    (println (p2 input))
    (shutdown-agents)))
