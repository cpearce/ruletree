from cffi import FFI
ffi = FFI()
ffi.cdef("""
  typedef struct{} RuleTree;
  typedef struct{} RuleMatches;
  RuleTree* rule_tree_new();
  void rule_tree_delete(RuleTree*);
  void rule_tree_insert(RuleTree*, int32_t*, size_t, int32_t);
  RuleMatches* rule_tree_matches(RuleTree*, int32_t*, size_t);
  size_t rule_matches_len(RuleMatches*);
  int32_t rule_matches_element(RuleMatches*, size_t index);
  void rule_matches_delete(RuleMatches*);
""")

lib = ffi.dlopen("target/debug/libpyffi.dylib")

tree = lib.rule_tree_new()
lib.rule_tree_insert(tree, [1, 2, 3, 4], 4, 1)
lib.rule_tree_insert(tree, [2, 3, 4], 3, 2)

matches = lib.rule_tree_matches(tree, [2,3], 2)
n = lib.rule_matches_len(matches)
print("{} matches".format(n))
for i in range(n):
  print(lib.rule_matches_element(matches, i))

lib.rule_matches_delete(matches)
lib.rule_tree_delete(tree)
