package util

type Set[T comparable] map[T]interface{}

func (s Set[T]) Add(v T) {
	s[v] = nil
}

func (s Set[T]) Delete(v T) {
	delete(s, v)
}

func (s Set[T]) Has(v T) bool {
	_, ok := s[v]
	return ok
}

func (s Set[T]) Union(t Set[T]) Set[T] {
	ret := make(Set[T])

	for k := range s {
		ret[k] = nil
	}

	for k := range t {
		ret[k] = nil
	}

	return ret
}

func (s Set[T]) Intersection(t Set[T]) Set[T] {
	ret := make(Set[T])
	var keys, other *Set[T]

	if len(s) <= len(t) {
		keys, other = &s, &t
	} else {
		keys, other = &t, &s
	}

	for k := range *keys {
		if other.Has(k) {
			ret[k] = nil
		}
	}

	return ret
}

func (s Set[T]) Difference(t Set[T]) Set[T] {
	ret := make(Set[T])

	for k := range s {
		if !t.Has(k) {
			ret.Add(k)
		}
	}

	return ret
}

func (s Set[T]) Without(v T) Set[T] {
	return s.Difference(Set[T]{v: nil})
}

func (s Set[T]) Slice() []T {
	slice := make([]T, 0, len(s))

	for k := range s {
		slice = append(slice, k)
	}

	return slice
}
