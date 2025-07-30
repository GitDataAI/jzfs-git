-- Database initialization script for Git Repository system
-- Generated from entity models
create extension IF NOT EXISTS "uuid-ossp";
-- Create users table
CREATE TABLE IF NOT EXISTS users (
    uid UUID PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    password VARCHAR(255) NOT NULL,
    email VARCHAR(100) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
    UNIQUE(username),
    UNIQUE(email)
);

-- Create repository table
CREATE TABLE IF NOT EXISTS repository (
    uid UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    owner UUID NOT NULL REFERENCES users(uid) ON DELETE CASCADE,
    description TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP,
    UNIQUE(name, owner)
);

-- Create git_branch table
CREATE TABLE IF NOT EXISTS git_branch (
    uid UUID PRIMARY KEY,
    repo_uid UUID NOT NULL REFERENCES repository(uid) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    head VARCHAR(40) NOT NULL,
    timestamp BIGINT NOT NULL,
    UNIQUE(repo_uid, name)
);

-- Create git_tags table
CREATE TABLE IF NOT EXISTS git_tags (
    uid UUID PRIMARY KEY,
    repo_uid UUID NOT NULL REFERENCES repository(uid) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    sha VARCHAR(40) NOT NULL,
    created_at TIMESTAMP NOT NULL,
    UNIQUE(repo_uid, name)
);

-- Create git_commit table
CREATE TABLE IF NOT EXISTS git_commit (
    uid UUID PRIMARY KEY,
    sha VARCHAR(40) NOT NULL,
    branch_uid UUID NOT NULL REFERENCES git_branch(uid) ON DELETE CASCADE,
    repo_uid UUID NOT NULL REFERENCES repository(uid) ON DELETE CASCADE,
    branch_name VARCHAR(100) NOT NULL,
    message TEXT NOT NULL,
    author_name VARCHAR(100) NOT NULL,
    author_email VARCHAR(100) NOT NULL,
    commiter_name VARCHAR(100) NOT NULL,
    commiter_email VARCHAR(100) NOT NULL,
    timestamp BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    UNIQUE(sha, repo_uid)
);

-- Create indexes for performance optimization
CREATE INDEX IF NOT EXISTS idx_repository_owner ON repository(owner);
CREATE INDEX IF NOT EXISTS idx_git_branch_repo_uid ON git_branch(repo_uid);
CREATE INDEX IF NOT EXISTS idx_git_tags_repo_uid ON git_tags(repo_uid);
CREATE INDEX IF NOT EXISTS idx_git_commit_branch_uid ON git_commit(branch_uid);
CREATE INDEX IF NOT EXISTS idx_git_commit_repo_uid ON git_commit(repo_uid);
CREATE INDEX IF NOT EXISTS idx_git_commit_timestamp ON git_commit(timestamp);