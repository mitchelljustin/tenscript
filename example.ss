(fabric
 (name "Headless Hug")
 (scale 105%)
 (features
  (iterations-per-frame 100)
  (push-over-pull 400%))
 (build
  (seed :left-right)
  (branch
   (grow A+
    (scale 95%)
    (twist 0 0 0 0 1 0 0)
    (mark :legs))
   (grow B+
    (scale 95%)
    (twist 0 0 0 0 1 0 0)
    (mark :legs))
   (grow A-
    (scale 90%)
    (branch
    (grow A+ 3
      (mark :shoulders))
    (grow C+
      (scale 93%)
      (twist 1 0 0 0 1 0 0)
      (mark :hands))))
   (grow B+
    (scale 90%)
    (branch
    (grow A+ 3
      (mark :shoulders))
    (grow C+
      (scale 93%)
      (twist 1 0 0 0 1 0 0)
      (mark :hands)))))
   (vulcanize :bowtie))
 (shape
  (pull-together :legs 5%)
  (pull-together :hands 7%)
  (pull-together :shoulders 5%))
 (pretense
  (wait 10000)
  (contract-conflicts)
  (wait 10000)
  (orient :legs)))