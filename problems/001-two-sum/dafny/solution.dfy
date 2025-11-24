datatype Option<T> = None | Some(value: T)


ghost predicate IsSolution(nums: seq<int>, target: int, i: nat, j: nat)
{
  i < |nums| && j < |nums| && i < j && nums[i] + nums[j] == target
}

ghost predicate ExistsSolution(nums: seq<int>, target: int)
{
  exists i, j :: IsSolution(nums, target, i, j)
}

ghost predicate ExistsUniqueSolution(nums: seq<int>, target: int)
{
  exists i, j :: IsSolution(nums, target, i, j) && forall k, l :: IsSolution(nums, target, k, l) ==> (k == i && l == j) || (k == j && l == i)
}

method twoSum_bf(nums: seq<int>, target: int) returns (result: Option<(nat, nat)>) 
    requires |nums| >= 2 && |nums| <= 10000
    requires forall x :: x in nums ==> 1000000000 <= x <= 1000000000
    requires target >= -1000000000 && target <= 1000000000
    requires ExistsUniqueSolution(nums, target)
    ensures result.Some?
    ensures var (i, j) := result.value; IsSolution(nums, target, i, j)
{
    var i := 0;
    while i < |nums| 
        invariant 0 <= i <= |nums| 
        invariant forall k, l :: 0 <= k < i && k < l < |nums| ==> nums[k] + nums[l] != target
    {
        var j := i + 1;
        while j < |nums| 
            invariant i < j <= |nums|
            invariant forall l :: i < l < j ==> nums[i] + nums[l] != target
        {
            if nums[i] + nums[j] == target 
            {
                return Some((i, j));
            }
            j := j + 1;
        }
        i := i + 1;
    }
    return None;
}


method twoSum_opt(nums: seq<int>, target: int) returns (result: Option<(nat, nat)>) 
    requires |nums| >= 2 && |nums| <= 10000
    requires forall x :: x in nums ==> 1000000000 <= x <= 1000000000
    requires target >= -1000000000 && target <= 1000000000
    requires ExistsUniqueSolution(nums, target)
    ensures result.Some?
    ensures var (i, j) := result.value; IsSolution(nums, target, i, j)
{
    var diffs : map<int, nat> := map[];
    var i := 0;
    while i < |nums|
        invariant 0 <= i <= |nums|
        invariant forall k :: 0 <= k < i ==> nums[k] in diffs
        invariant forall k :: k in diffs ==> diffs[k] < i && nums[diffs[k]] == k
        invariant forall k, l :: 0 <= k < l < i ==> nums[k] + nums[l] != target
    {
        var complement := target - nums[i];
        if complement in diffs 
        {
            return Some((diffs[complement], i));
        }
        diffs := diffs[nums[i] := i];
        i := i + 1;
    }
    return None;
}
