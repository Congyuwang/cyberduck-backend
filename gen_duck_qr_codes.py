import os
import sys
import requests
import argparse
# pip install "qrcode[pil]"
from PIL import Image
import qrcode
from pathlib import Path
from urllib.parse import urlparse
from io import BytesIO

ALL_DUCKS_ENDPOINT = "/admin/many-ducks"
OUTPUT_DIR = "./qrcodes"


def ducks_endpoint(frontend, id_of_duck):
    url = urlparse(f"{frontend}/duck/{id_of_duck}")
    return url.geturl()


def bearer_header(token):
    return {"Authorization": f"Bearer {token}"}


def request_all_ducks(base_url, token):
    url = urlparse(base_url + ALL_DUCKS_ENDPOINT)
    rsp = requests.get(
        url=url.geturl(),
        headers=bearer_header(token),
        verify=False,
    )
    if rsp.status_code == 400:
        print("invalid admin token")
        sys.exit(-1)
    else:
        return rsp.json()


def large_icon(icon_url: str):
    path, name = icon_url.rsplit("/", maxsplit=1)
    return "/".join([path, f"3x-{name}"])


def duck_info(duck_data):
    return {
        "id": duck_data["id"],
        "name": duck_data["title"]["cn"],
        "loc": duck_data["location"]["description"]["cn"],
        "icon": large_icon(duck_data["duckIconUrl"]),
    }


def get_icon(icon_url):
    rsp = requests.get(icon_url)
    return Image.open(BytesIO(rsp.content))


if __name__ == '__main__':
    parser = argparse.ArgumentParser("generate QR codes of the ducks", allow_abbrev=True)
    parser.add_argument("-e, --endpoint", dest="endpoint", type=str, help="backend base url")
    parser.add_argument("-t, --token", dest="token", type=str, help="admin token")
    parser.add_argument("-f, --frontend-url", dest="frontend", type=str, help="frontend base url")
    args = parser.parse_args(args=sys.argv[1:])
    ducks = [duck_info(dd) for dd in request_all_ducks(args.endpoint, args.token)]
    os.makedirs(OUTPUT_DIR, exist_ok=True)
    for duck in ducks:
        qr = qrcode.QRCode(
            box_size=15,
            border=2,
            error_correction=qrcode.ERROR_CORRECT_H
        )
        duck_url = ducks_endpoint(args.frontend, duck["id"])
        print(duck_url)
        qr.add_data(duck_url)
        qr.make(fit=True)
        qr_img = qr.make_image()
        icon = get_icon(duck["icon"])
        pos = int(qr_img.pixel_size / 2 - icon.width / 2)
        Image.Image.paste(qr_img, icon, (pos, pos))
        with open(Path(OUTPUT_DIR) / f"{duck['name']}({duck['loc']}).png", "wb") as fp:
            qr_img.save(fp)
