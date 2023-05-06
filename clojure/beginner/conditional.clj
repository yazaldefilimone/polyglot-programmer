(defn isPair? [dividend]
  (if (even? dividend) "pair" "odd"))

(isPair? 2) ;; pair
(isPair? 5) ;; odd

(defn isPosetiveNumber [num]
  (cond (> num  0)  "posetive" (< num  0) "negative" :else "zero"))

(isPosetiveNumber 0) ; zero