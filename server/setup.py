from setuptools import setup, find_packages

setup(
    name="librectf",
    version="0.1.0",
    packages=find_packages(),
    include_package_data=True,
    zip_safe=False,
    install_requires=["Flask>=2.0"],
    entry_points={
        "console_scripts": ["librectf_server=librectf:create_app"],
    },
)
