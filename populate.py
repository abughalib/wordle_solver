import sqlite3
from bs4 import BeautifulSoup

con = sqlite3.connect("words.db")

cur = con.cursor()


def define_sql():
    # Create table
    con.execute(
        '''
        CREATE TABLE IF NOT EXISTS words(
            id integer primary key AUTOINCREMENT, word varchar(255), word_length integer,
            UNIQUE(word)
        );
        '''
    )

    # Create Secondary Index
    con.execute(
        '''
        CREATE INDEX IF NOT EXISTS word_length ON words(word_length)
        '''
    )


def insert_word(word: str, word_length: int):
    try:
        con.execute(
            f'''
        INSERT INTO words(word, word_length) VALUES(?, ?);
        ''', (word.lower(), word_length)
        )
    except:
        print(f"Some Error while inserting: {word}")


define_sql()


def scrap_file(file):
    soup = BeautifulSoup(file, features='lxml')

    dup_check = []

    for line in soup.find_all('p'):
        for word in line.find_next('b'):

            if word.__contains__("'") or word.__contains__('-') or word.__contains__(' '):
                continue

            if len(dup_check) == 0:
                dup_check.append(word)
            else:
                if word == dup_check[0]:
                    continue
                else:
                    dup_check.pop()
                    dup_check.append(word)
            print("Inserting {}".format(word))
            insert_word(word, len(word))

    file.close()


def scrap_word():
    for alp in range(ord('a'), ord('z') + 1):
        file = open(f"words/{chr(alp)}.html", mode='r')
        scrap_file(file)
        file.close()
    file = open('words/new.html', mode='r')
    scrap_file(file)
    file.close()


scrap_word()

con.commit()
con.close()