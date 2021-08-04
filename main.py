import io
import creds
import pickle


class Credentials:
    def __init__(self, username: str, password: str, permissions: str):
        self.username = username
        self.password = password
        self.permissions = permissions

if __name__ == "__main__":
    usr, pwd, prm = ('user', 'pass', 'ReadOnly')
    py_creds = Credentials(usr, pwd, prm)
    rs_creds = creds.Credentials(usr, pwd, prm)

    buffer = io.BytesIO()
    pickle.dump(py_creds, buffer)
    pickle.dump(rs_creds, buffer)

