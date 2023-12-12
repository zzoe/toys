DEFINE TABLE user SCHEMAFULL PERMISSIONS FOR select, update, delete WHERE id = $auth.id;
DEFINE FIELD name ON user TYPE string;
DEFINE FIELD email ON user TYPE string ASSERT string::is::email($value);
DEFINE FIELD password ON user TYPE string;
DEFINE INDEX email ON user FIELDS email UNIQUE;
DEFINE SCOPE user_scope SESSION 10h
    SIGNUP (
        CREATE user CONTENT {
            name: $name,
            email: $email,
            password: crypto::argon2::generate($password)
        }
    )
    SIGNIN ( SELECT * FROM user WHERE email = $email AND crypto::argon2::compare(password, $password) );