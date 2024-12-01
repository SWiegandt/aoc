(ns day4
  (:require
   [util]
   [clojure.string :as str]))

(defn parse [input]
  (->> input
       (#(str/split % #"\n\n"))
       (map (fn [passport]
              (->> passport
                   (#(str/split % #"\s+"))
                   (map #(str/split % #":"))
                   (into {}))))))

(defn between? [start stop n] (and n (>= n start) (<= n stop)))

(defn valid? [{byr "byr"
               iyr "iyr"
               eyr "eyr"
               hgt "hgt"
               hcl "hcl"
               ecl "ecl"
               pid "pid"}]
  (let [byr (some-> byr parse-long)
        iyr (some-> iyr parse-long)
        eyr (some-> eyr parse-long)]
    (and
     (between? 1920 2022 byr)
     (between? 2010 2020 iyr)
     (between? 2020 2030 eyr)
     (let [matcher (some->> hgt (re-matcher #"(\d+)(cm|in)"))
           [_ hgt unit] (some-> matcher re-find)]
       (case unit
         "cm" (between? 150 193 (parse-long hgt))
         "in" (between? 59 76 (parse-long hgt))
         nil))
     (some->> hcl (re-matches #"#[0-9a-f]{6}"))
     (some #(= ecl %) ["amb" "blu" "brn" "gry" "grn" "hzl" "oth"])
     (some->> pid (re-matches #"\d{9}")))))

(defn p1 [input]
  (->> input
       parse
       (filter #(every? % ["byr" "iyr" "eyr" "hgt" "hcl" "ecl" "pid"]))
       count))

(defn p2 [input]
  (->> input parse (filter valid?) count))

(defn -main []
  (util/download-input 4)
  (let [input (util/read-input 4)]
    (println (p1 input))
    (println (p2 input))
    (shutdown-agents)))
