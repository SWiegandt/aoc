(ns util
  (:require
   [clojure.java.shell :as shell]
   [clojure.string :as str]))

(defn download-input [day]
  (shell/sh "../get-input" "2020" (str day)))

(defn read-input [day]
  (slurp (str "../inputs/2020/" day ".txt")))

(defn input-list
  ([input parse]
   (-> input (str/split #"\n") (#(map parse %))))
  ([input] (input-list input identity)))
