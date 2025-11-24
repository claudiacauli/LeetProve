import Std.Data.HashMap


-- Guaranteed to terminate (or Lean would not have type-checked it_)
def twoSum_bf (nums: Array Int) (target: Int) : Option (Nat × Nat) :=
  (List.range nums.size).findSome? fun i =>
    (List.range nums.size).findSome? fun j =>
      if i < j && nums[i]! + nums[j]! = target then
        some (i, j)
      else
        none


-- Guaranteed to terminate (or Lean would not have type-checked it_)
def twoSum_opt (nums : Array Int) (target : Int) : Option (Nat × Nat) :=
  Id.run do
  let mut map : Std.HashMap Int Nat := {}
  for i in [0:nums.size] do
    let num := nums[i]!
    let diff := target - num
    if let some j := map[diff]? then
      return some (j, i)
    map := map.insert num i
  return none


def ValidArraySize (nums : Array Int) : Prop :=
  2 ≤ nums.size ∧ nums.size ≤ 10^4

def ValidNums (nums : Array Int) : Prop :=
  ∀ x ∈ nums, -10^9 ≤ x ∧ x ≤ 10^9

def ValidTarget (target : Int) : Prop :=
  -10^9 ≤ target ∧ target ≤ 10^9

def IsSolution(num: Array Int) (target: Int) (i j: Nat) : Prop :=
  i < num.size ∧ j < num.size ∧ i < j ∧ num[i]! + num[j]! = target

def ExistsASolution(nums: Array Int) (target: Int) : Prop :=
  ∃ (i j : Nat), IsSolution nums target i j

def ExistsOneSolution(nums: Array Int) (target: Int) : Prop :=
  ∃ (i j : Nat), IsSolution nums target i j
  ∧ ∀ (k l : Nat), IsSolution nums target k l → (k = i ∧ l = j) ∨ (k = j ∧ l = i)


theorem twoSum_bf_returns_option (nums: Array Int) (target: Int) :
  (twoSum_bf nums target).isSome ∨ (twoSum_bf nums target).isNone := by
  cases twoSum_bf nums target <;> simp

theorem twoSum_opt_returns_option (nums: Array Int) (target: Int) :
  (twoSum_opt nums target).isSome ∨ (twoSum_opt nums target).isNone := by
  cases twoSum_opt nums target <;> simp
