char * defangIPaddr(char * address){
    int n = strlen(address);
    int m = n+6+1;
    char* ret = (char*)calloc(m, sizeof(char));
    while(1) {
        char* p = strstr(address, ".");
        if(!p) {
            strcat(ret, address);
            break;
        }
        int n = p - address;
        strncat(ret, address, n);
        strcat(ret, "[.]");
        address = p+1;
    }
    return ret;
}
