# remove-alpha

Removes all Alpha bytes from a binary with Red, Green, Blue and Alpha color channels.

## Before
| r  | g  | b  | a  | r  | g  | b  | a  |...|
|----|----|----|----|----|----|----|----|---|
| 50 | 8e | a5 | ff | 53 | 90 | a5 | ff |...|

## After
| r  | g  | b  | r  | g  | b  |...|
|----|----|----|----|----|----|---|
| 50 | 8e | a5 | 53 | 90 | a5 |...|
