from setuptools import setup, find_packages

with open("README.md", "r", encoding="utf-8") as fh:
    long_description = fh.read()

setup(
    name="magneto-pytest",
    version="1.0.0",
    author="Magnéto-Serge Contributors",
    description="Pytest plugin for Magnéto-Serge cassette testing",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/your-org/magneto-serge",
    packages=find_packages(),
    classifiers=[
        "Development Status :: 4 - Beta",
        "Framework :: Pytest",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Software Development :: Testing",
    ],
    python_requires=">=3.8",
    install_requires=[
        "pytest>=6.0.0",
    ],
    entry_points={
        "pytest11": [
            "magneto = magneto_pytest",
        ],
    },
)
