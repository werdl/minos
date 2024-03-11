(load "/lib/lisp/alias.lsp")

(def (string? x)
  (equal? (type x) "string"))

(def (boolean? x)
  (equal? (type x) "boolean"))

(def (symbol? x)
  (equal? (type x) "symbol"))

(def (number? x)
  (equal? (type x) "number"))

(def (list? x)
  (equal? (type x) "list"))

(def (function? x)
  (equal? (type x) "function"))

(def (macro? x)
  (equal? (type x) "macro"))

(var nil '())

(def (nil? x)
  (equal? x nil))

(def (not x)
  (if x false true))

(def-mac (or x y)
  `(if ,x ,x (if ,y ,y false)))

(def-mac (and x y)
  `(if ,x (if ,y true false) false))

(def (empty? x)
  (or
    (equal? x nil)
    (equal? x "")))

(def-mac (let params values body)
  `((fun ,params ,body) ,@values))

(def (reduce f ls)
  "Reduce the elements of the list with the function"
  (if (nil? (tail ls)) (head ls)
    (f (head ls) (reduce f (tail ls)))))

(def (map f ls)
  "Apply the function to the elements of the list"
  (if (nil? ls) nil
    (cons
      (f (head ls))
      (map f (tail ls)))))

(def (filter f ls)
  "Filter the elements of the list with the function"
  (if (nil? ls) nil
    (if (f (head ls))
      (cons (head ls) (filter f (tail ls)))
      (filter f (tail ls)))))

(def (reject f ls)
  "Reject the elements of the list with the function"
  (if (nil? ls) nil
    (if (not (f (head ls)))
      (cons (head ls) (reject f (tail ls)))
      (reject f (tail ls)))))

(def (intersection a b)
  "Return elements found in both lists"
  (filter (fun (x) (contains? b x)) a))

(def (reverse ls)
  "Reverse list"
  (if (nil? ls) ls
    (concat (reverse (tail ls)) (cons (head ls) '()))))

(def (range start stop)
  "Return a list of integers from start to stop excluded"
  (if (= start stop) nil
    (concat (list start) (range (+ start 1) stop))))

(def (min ls)
  "Return the minimum element of the list"
  (head (sort ls)))

(def (max ls)
  "Return the maximum element of the list"
  (head (reverse (sort ls))))

(def (abs x)
  (if (> x 0) x (- x)))

(def (mod a b)
  (rem (+ (rem a b) b) b))

(def (string/join ls s)
  "Join the elements of the list with the string"
  (reduce (fun (x y) (string x s y)) ls))

(def (regex/match? r s)
  "Return true if the string match the pattern"
  (not (nil? (regex/find r s))))

(def (lines text)
  "Split text into a list of lines"
  (string/split (string/trim text) "\n"))

(def (words text)
  "Split text into a list of words"
  (string/split text " "))

(def (chars text)
  "Split text into a list of chars"
  (string/split text ""))

(def (push ls x)
  "Push element to the end of the list"
  (put ls (len ls) x))

(def (first ls)
  (get ls 0))

(def (second ls)
  (get ls 1))

(def (third ls)
  (get ls 2))

(def (last ls)
  (get ls
    (if (= (length ls) 0) 0 (- (length ls) 1))))

# Short aliases

(var sh shell)
(var % rem)
(var str string)
(var str/split string/split)
(var str/join string/join)
(var str/trim string/trim)
(var num/type number/type)
(var str->num string->number)
(var str->bin string->binary)
(var num->bin number->binary)
(var num->str number->string)
(var bin->str binary->string)
(var bin->num binary->number)
(var bool? boolean?)
(var str? string?)
(var sym? symbol?)
(var num? number?)
(var fun? function?)
(var mac? macro?)
(var len length)
(var rev reverse)
(var uniq unique)

(load "/lib/lisp/file.lsp")
