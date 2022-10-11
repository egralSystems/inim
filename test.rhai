for file in ls(".") {
    print(file);
}

print("==============");
print("Year:" + ((time()/31556926) + 1970));
print("==============");

print(path());