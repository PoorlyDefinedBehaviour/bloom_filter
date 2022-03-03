# Bloom filter

A bloom filter is a data-structure that can be used to check if a set contains an element. It uses way less memory than a [conventional set data-structure](https://en.wikipedia.org/wiki/Set_(abstract_data_type)#Implementations) by sacrificing accuracy.

Say we are building a [log-structured merge-tree](https://www.cs.umb.edu/~poneil/lsmtree.pdf), we can use a bloom filter to find out if the LSM-tree contains a particular key in O(1) time in most cases.

# References

Network Applications of Bloom Filters: A Survey - https://www.eecs.harvard.edu/~michaelm/postscripts/im2005b.pdf
