
; ex1.1


; ex1.2
(defun power (x n)
  (if (= n 0)
      1
      (* x (power x (1- n)))))

; ex1.3
(defun count-atoms (exp)
  (cond ((null exp) 0)
	((atom exp) 1)
	(t (+ (count-atoms (first exp))
	      (count-atoms (rest exp))))))

