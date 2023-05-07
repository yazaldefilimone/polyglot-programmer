(def age 18)
(def firstName "Yazalde")
(def height  1.70)

(println (str "Age: " age ", Fist Name: " firstName ", and Height: " height))

(def rationalNumber 1/3) ; rational
(def is true) ; boolean
(def isNot false) ; boolean
(def iamNull nil) ; nil (null)

(println rationalNumber)
(println is)
(println isNot)
(println (str "= iamNull isNot: " (= iamNull isNot)))

(def iamList '(1,2,3,4,5,6)) ; list (array )
(def iamVector [1,2,3,4,5,6]) ; vectors (array fix size)
(def iamMap { :age 17, :firstName "yazalde", :height 1.70}) ; map(object)
(def iamCuj #{1,2,3,4,5}) ; sets (collection of unique data)


(println iamList)
(println iamVector)
(println iamMap)
(println iamCuj)