import { Client } from 'pg';

const getAuthorQuery = `-- name: GetAuthor :one
SELECT * FROM authors
WHERE id = $1 LIMIT 1;`

export type GetAuthorParams = {
    id: bigint | null;
}

export type GetAuthorRow = {
    id: bigint;
    name: string;
    bio: string | null;
}

export async function getAuthor(client: Client, args: GetAuthorParams): Promise<GetAuthorRow | null> {
  const result = await client.query({
    text: getAuthorQuery,
    values: [args.id],
    rowMode: 'array',
  })
  if (result.rows.length !== 1) {
    return null
  }
  const row = result.rows[0]
  return {
    id: row[0],
    name: row[1],
    bio: row[2],
  }
}

const listAuthorsQuery = `-- name: ListAuthors :many
SELECT * FROM authors
ORDER BY name;
`

export type ListAuthorsRow = {
    id: bigint;
    name: string;
    bio: string | null;
}


export async function listAuthors(client: Client): Promise<ListAuthorsRow[]> {
  const result = await client.query({
    text: listAuthorsQuery,
    rowMode: 'array',
  })
  return result.rows.map(row => {
    return {
      id: row[0],
      name: row[1],
      bio: row[2],
    }
  })
}

const createAuthorQuery = `-- name: CreateAuthor :one
INSERT INTO authors (
  name, bio
) VALUES (
  $1, $2
)
RETURNING *;`

export type CreateAuthorParams = {
    name: string | null;
    bio: string | null;
}

export type CreateAuthorRow = {
    id: bigint;
    name: string;
    bio: string | null;
}

export async function createAuthor(client: Client, args: CreateAuthorParams): Promise<CreateAuthorRow | null> {
  const result = await client.query({
    text: createAuthorQuery,
    values: [args.name, args.bio],
    rowMode: 'array',
  })
  if (result.rows.length !== 1) {
    return null
  }
  const row = result.rows[0]
  return {
    id: row[0],
    name: row[1],
    bio: row[2],
  }
}

const deleteAuthorQuery = `-- name: DeleteAuthor :exec
DELETE FROM authors
WHERE id = $1;`

export type DeleteAuthorParams = {
    id: bigint | null;
}

export async function deleteAuthor(client: Client, args: DeleteAuthorParams): Promise<void> {
  await client.query(deleteAuthorQuery, [args.id])
}

const deleteAllAuthorsQuery = `-- name: DeleteAllAuthors :exec
DELETE FROM authors
`

export async function deleteAllAuthors(client: Client): Promise<void> {
  await client.query(deleteAllAuthorsQuery);
}