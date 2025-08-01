package flatten

func Flatten(nested interface{}) []interface{} {
    ret := []interface{}{}
    switch nested.(type) {
        case []interface{}:
        	for _, x := range(nested.([]interface{})) {
                if x != nil {
                    ret = append(ret, Flatten(x)...)
                }
            }
        case int:
        	ret = append(ret, nested.(int))
    }
    return ret
}
