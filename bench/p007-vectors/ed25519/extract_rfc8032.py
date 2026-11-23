"""Transcribe the Ed25519 test vectors of RFC 8032 section 7.1.

Python standard library only. Usage:

    python -I extract_rfc8032.py <path to rfc8032.txt> <output file>

The RFC text is not committed. Fetch https://www.rfc-editor.org/rfc/rfc8032.txt; this
script refuses any file whose SHA-256 is not the one recorded below, so the committed
transcription can be reproduced byte for byte. derive.py then checks the transcription
a second way: it recomputes every public key and signature from the secret key with its
own RFC 8032 implementation.
"""

import hashlib
import re
import sys

RFC_SHA256 = "ed63657ff389301282b169b0abde9b5dd2c7e4d524fdfa5da6ff3094fc93c4c3"
IDS = {"1": "1", "2": "2", "3": "3", "1024": "1024", "SHA(abc)": "sha-abc"}
HEX_LINE = re.compile(r"^[0-9a-f]+$")


def main(argv):
    if len(argv) != 3:
        sys.stderr.write(__doc__)
        return 2
    raw = open(argv[1], "rb").read()
    digest = hashlib.sha256(raw).hexdigest()
    if digest != RFC_SHA256:
        sys.stderr.write("unexpected RFC text: sha256 %s\n" % digest)
        return 1
    lines = raw.decode("ascii").splitlines()
    start = lines.index("7.1.  Test Vectors for Ed25519")
    end = lines.index("7.2.  Test Vectors for Ed25519ctx")

    tests = []
    current = None
    field = None
    for line in lines[start:end]:
        text = line.strip()
        if text.startswith("-----TEST "):
            current = {"id": IDS[text[len("-----TEST ") :]]}
            tests.append(current)
            field = None
        elif current is None:
            continue
        elif text == "ALGORITHM:":
            field = "algorithm"
            current[field] = ""
        elif text == "SECRET KEY:":
            field = "secret_key"
            current[field] = ""
        elif text == "PUBLIC KEY:":
            field = "public_key"
            current[field] = ""
        elif text.startswith("MESSAGE (length "):
            field = "message"
            current[field] = ""
            current["message_length"] = int(text.split()[2])
        elif text == "SIGNATURE:":
            field = "signature"
            current[field] = ""
        elif field == "algorithm" and text:
            current[field] += text
        elif field is not None and HEX_LINE.match(text):
            # Page headers and footers are not lower-case hex and are skipped.
            current[field] += text

    assert [test["id"] for test in tests] == list(IDS.values()), tests
    out = [
        "# RFC 8032 section 7.1, test vectors for Ed25519: a transcription.",
        "# Source: https://www.rfc-editor.org/rfc/rfc8032.txt, SHA-256",
        "# %s." % RFC_SHA256,
        "# Written by extract_rfc8032.py. Do not edit by hand.",
        "",
    ]
    for test in tests:
        assert test["algorithm"] == "Ed25519", test
        assert len(test["secret_key"]) == 64 and len(test["public_key"]) == 64, test
        assert len(test["signature"]) == 128, test
        assert len(test["message"]) == 2 * test["message_length"], test
        prefix = "rfc8032.%s." % test["id"]
        for name in ("secret_key", "public_key", "message", "signature"):
            # An empty value (the empty message of TEST 1) leaves nothing after "=".
            out.append((prefix + name + " = " + test[name]).rstrip())
        out.append("")
    with open(argv[2], "w", encoding="ascii", newline="\n") as handle:
        handle.write("\n".join(out))
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
