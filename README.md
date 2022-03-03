# Bloom filter

A bloom filter is a data-structure that can be used to check if a set contains an element. It sacrifices accuracy to use way less memory than a conventional set.

Say we are building a [log-structured merge-tree](https://www.cs.umb.edu/~poneil/lsmtree.pdf), we can use a bloom filter to find out if the LSM-tree contains a particular key in O(1) time in most cases.

that answers `definitely not` or `probably yes` when asked the question `do you contain element x?`.

# References

Network Applications of Bloom Filters: A Survey - https://www.eecs.harvard.edu/~michaelm/postscripts/im2005b.pdf
