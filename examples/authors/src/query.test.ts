import { describe, beforeEach, afterEach, expect, test} from '@jest/globals';

import { Client } from 'pg'

import { createAuthor, deleteAllAuthors, deleteAuthor, getAuthor, listAuthors } from './query.sql';

describe('query functions', () => {
  let client = new Client()

  beforeEach(async () => {
    client = new Client()
    await client.connect()
    await deleteAllAuthors(client)
  });

  afterEach(async () => {
    await client.end()
  });
   
  test('createAuthor', async () => {
    const created = await createAuthor(client, {
      name: 'Ernest Hemmingway',
      bio: 'An angry drunk who loved cats',
    })
    if (created === null ) {
        fail("createAuthor returned null")
    }

    const fetched = await getAuthor(client, created)
    if (fetched === null ) {
        fail("getAuthor returned null")
    }
    expect(fetched).toStrictEqual(created)

    const list = await listAuthors(client)
    expect(list).toStrictEqual([created])

    await deleteAuthor(client, created)

    const refetched = await getAuthor(client, created)
    expect(refetched).toBeNull()
  });
});