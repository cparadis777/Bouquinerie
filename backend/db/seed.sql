-- ── Seed data for ebook library ────────────────────────────────────────────
-- Run: task db:seed

-- Authors ────────────────────────────────────────────────────────────────────
INSERT INTO authors (id, name, sort_name) VALUES
  ('00000000-0000-0000-0000-000000000001', 'J.R.R. Tolkien', 'Tolkien, J.R.R.'),
  ('00000000-0000-0000-0000-000000000002', 'George Orwell', 'Orwell, George'),
  ('00000000-0000-0000-0000-000000000003', 'Aldous Huxley', 'Huxley, Aldous'),
  ('00000000-0000-0000-0000-000000000004', 'Frank Herbert', 'Herbert, Frank');

-- Series ─────────────────────────────────────────────────────────────────────
INSERT INTO series (id, name, description, created_at, updated_at) VALUES
  ('00000000-0000-0000-0000-000000000010', 'The Lord of the Rings',
   'Epic high fantasy trilogy', NOW(), NOW());

-- Books ──────────────────────────────────────────────────────────────────────
INSERT INTO books (id, title, subtitle, description, language, publisher, isbn,
                   page_count, cover_path, published_date, created_at, updated_at)
VALUES
  ('00000000-0000-0000-0000-000000000020', 'The Hobbit', '',
   'A reluctant hobbit embarks on a perilous adventure.', 'en',
   'Houghton Mifflin', '978-0547928227', 310, '', '1937-09-21', NOW(), NOW()),
  ('00000000-0000-0000-0000-000000000021', 'The Fellowship of the Ring',
   'The Lord of the Rings, Part 1',
   'A young hobbit inherits a ring of immense power.', 'en',
   'Houghton Mifflin', '978-0547928210', 423, '', '1954-07-29', NOW(), NOW()),
  ('00000000-0000-0000-0000-000000000022', '1984', '',
   'A dystopian novel set in a totalitarian society.', 'en',
   'Secker & Warburg', '978-0451524935', 328, '', '1949-06-08', NOW(), NOW()),
  ('00000000-0000-0000-0000-000000000023', 'Brave New World', '',
   'A futuristic society where humans are genetically modified.', 'en',
   'Chatto & Windus', '978-0060850524', 311, '', '1932-01-01', NOW(), NOW()),
  ('00000000-0000-0000-0000-000000000024', 'Dune', '',
   'The epic story of Paul Atreides on the desert planet Arrakis.', 'en',
   'Chilton Books', '978-0441172719', 412, '', '1965-08-01', NOW(), NOW());

-- Join: series_books ─────────────────────────────────────────────────────────
INSERT INTO series_books (book_id, series_id, position, sort_order) VALUES
  ('00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000010', '1', 1);

-- Join: authors_books ─────────────────────────────────────────────────────────
INSERT INTO authors_books (book_id, author_id, sort_order) VALUES
  ('00000000-0000-0000-0000-000000000020', '00000000-0000-0000-0000-000000000001', 1),
  ('00000000-0000-0000-0000-000000000021', '00000000-0000-0000-0000-000000000001', 1),
  ('00000000-0000-0000-0000-000000000022', '00000000-0000-0000-0000-000000000002', 1),
  ('00000000-0000-0000-0000-000000000023', '00000000-0000-0000-0000-000000000003', 1),
  ('00000000-0000-0000-0000-000000000024', '00000000-0000-0000-0000-000000000004', 1);

-- Identifiers ────────────────────────────────────────────────────────────────
INSERT INTO identifiers (id, book_id, source, value) VALUES
  (gen_random_uuid(), '00000000-0000-0000-0000-000000000020', 'google_books', 'HobbitGoogleId'),
  (gen_random_uuid(), '00000000-0000-0000-0000-000000000022', 'google_books', '1984GoogleId'),
  (gen_random_uuid(), '00000000-0000-0000-0000-000000000024', 'google_books', 'DuneGoogleId');

-- Done ───────────────────────────────────────────────────────────────────────
SELECT COUNT(*) || ' books seeded.' AS result FROM books;
