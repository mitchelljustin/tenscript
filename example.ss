(fabric
 (name "Knee")
 (surface :bouncy)
 (features
  (iterations-per-frame 100)
  (gravity 150%))
 (build
  (seed :left)
  (branch
   (grow A+ 3
    (mark A+ :arm)
    (mark B- :leg)
    (branch
     (grow A+ 3)))
   (grow B- 3
    (mark A+ :arm)))
  (vulcanize :bowtie)))
