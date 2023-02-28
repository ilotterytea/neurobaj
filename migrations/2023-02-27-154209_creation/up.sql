-- Your SQL goes here
CREATE TABLE
  "chains" (
    "id" INTEGER NOT NULL,
    "from_word" TEXT NOT NULL DEFAULT '\\x02',
    "to_word" TEXT NOT NULL DEFAULT '\\x03',
    "from_word_signature_id" INTEGER NOT NULL,
    "to_word_signature_id" INTEGER NOT NULL,
    "msg_id" TEXT NOT NULL,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );

CREATE TABLE
  "signatures" (
    "id" INTEGER NOT NULL,
    "channel_id" INTEGER NOT NULL,
    "sender_id" INTEGER NOT NULL,
    "is_ignored" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );

CREATE TABLE
  "channels" (
    "id" INTEGER NOT NULL,
    "alias_id" INTEGER NOT NULL,
    "platform_id" INTEGER NOT NULL,
    "channel_name" INTEGER NOT NULL,
    "ignore_phrases" TEXT,
    PRIMARY KEY ("id" AUTOINCREMENT)
  );
