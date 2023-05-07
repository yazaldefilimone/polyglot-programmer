(def listOfNumber '(1,2,3,4,5,6,7,8,9,10))

;; (for [item listOfNumber]
;; (println item)
;; )

(for [item '(1 2 3)] (println item))
(println (map inc listOfNumber))
