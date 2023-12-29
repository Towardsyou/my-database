1. role based authorization
    1. dba
    2. normal user

Create issue
1. A create an issue
2. B approve an issue
3. A/B apply the issue

Issue history
1. starter, approver, status


adhoc editor
1. write sql
2. execute sql
3. save notebook
4. select goes without approval, delete/update/alter require approval

4. executed selected

manage connections
1. CRUD on connections


```
struct Issue {
    id: i32,
    name: String,
    description: String,
    status: IssueStatus,
    approver: Vec<String>,
    creator: String,
    created_at: DateTime,
    updated_at: DateTime,
}

trait IssueService {
    fn create_issue(&self, issue: Issue) -> Result<Issue, Error>;
    fn close_issue(&self, issue: Issue) -> Result<Issue, Error>;
    fn get_issue(&self, id: i32) -> Result<Issue, Error>;
    fn get_issues(&self, query: IssueQuery) -> Result<Vec<Issue>, Error>;
}
```

```
struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
    role: Role,
    status: UserStatus,
    created_at: DateTime,
}

trait UserService {
    fn create_user(&self, user: User) -> Result<User, Error>;
    fn update_user(&self, user: User) -> Result<User, Error>;
    fn get_user(&self, id: i32) -> Result<User, Error>;
    fn get_users(&self, query: UserQuery) -> Result<Vec<User>, Error>;
}
```

```
trait DatabaseService {
    fn execute_sql(&self, sql: String) -> Result<ResultSet, Error>;
}
```