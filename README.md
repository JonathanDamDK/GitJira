# GitJira
A cli to automatically detect and prepend jira ticket numbers to your commits
# building
To build run the following command
```
cargo build -r 
```

# usage
After adding the build to your path, you can use git to add and push the files, and gj to commit, the following is a full example
```
git add .
gj commit -m "Added unit tests"
git push
``` 
gj will automaticly register the branch name, and prepend the project name and ticket number to the commit.
