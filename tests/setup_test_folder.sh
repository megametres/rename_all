#!/bin/bash
#Cleanup previous test
rm -rf sample_path test_path

# Simple file to generates files and folder to test

mkdir -p sample_path/sample_parent_a/sample_child_a1
mkdir -p sample_path/sample_parent_a/sample_child_a2
mkdir -p sample_path/SAMPLE_parent_b/SAMPLE_child_b1
mkdir -p sample_path/Sample_parent_c/Sample_child_c1

touch sample_path/sample_parent_a/sample_child_a1/sample_file_a1
touch sample_path/sample_parent_a/sample_child_a2/sample_file_a2
touch sample_path/SAMPLE_parent_b/SAMPLE_child_b1/SAMPLE_file_b
touch sample_path/Sample_parent_c/Sample_child_c1/Sample_file_c