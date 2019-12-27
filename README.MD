Day 71: Hash Table — chaining
<br>

In hashing there is a hash function that maps keys to some values. But these hashing function may lead to collision that is two or more keys are mapped to same value. Chain hashing avoids collision. The idea is to make each cell of hash table point to a linked list of records that have same hash function value.
<br>
Let’s create a hash function, such that our hash table has ‘N’ number of buckets.
To insert a node into the hash table, we need to find the hash index for the given key. And it could be calculated using the hash function.
Example: hashIndex = key % noOfBuckets
<br>
Insert: Move to the bucket corresponds to the above calculated hash index and insert the new node at the end of the list.
<br>
Delete: To delete a node from hash table, calculate the hash index for the key, move to the bucket corresponds to the calculated hash index, search the list in the current bucket to find and remove the node with the given key (if found).
<br>

run:
```

fn main() {
    let n1 = new_node!("1");
    let n2 = new_node!("2");
    let n3 = new_node!("3");
    let n4 = new_node!("4");
    let n5 = new_node!("5");

    let n3d = new_node!("3");
    let mut hash_chain: HashChainNode = HashChainNode::default();
    hash_chain.insert(n1);
    hash_chain.insert(n2);
    hash_chain.insert(n3);
    hash_chain.insert(n4);
    hash_chain.insert(n5);
    println!("{}", hash_chain);
    hash_chain.delete(&n3d);
    println!("{}", hash_chain);
}
```

result:
```
1:3 
3:4 
7:2 5 
6:1 

1:
3:4 
7:2 5 
6:1 

```