#!/bin/sh

GEN_TEST() {
	cat target_template.txt | sed s/TARGET_NAME/$1/g > $1_target.rs
}

GEN_TEST url_parse
