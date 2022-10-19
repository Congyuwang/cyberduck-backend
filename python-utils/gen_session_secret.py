import secrets
import base64
import sys


def new_key(length: int):
    return base64.b64encode(secrets.token_bytes(length)).decode("utf-8")


def url_safe_token(length: int):
    return secrets.token_urlsafe(length)


if __name__ == '__main__':
    if len(sys.argv) >= 3 and sys.argv[2] == "url":
        print(url_safe_token(int(sys.argv[1])))
    else:
        print(new_key(int(sys.argv[1])))
