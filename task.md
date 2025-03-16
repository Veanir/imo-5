# Dokument

## Strona 1

### Opis problemu optymalizacji
Podczas zajęć laboratoryjnych będziemy wykorzystywać zmodyfikowany problem komiwojażera. Dany jest zbiór wierzchołków i symetryczna macierz odległości pomiędzy dowolną parą wierzchołków. Należy ułożyć dwie rozłączne zamknięte ścieżki (cykle), każda zawierająca 50% wierzchołków (jeżeli liczba wierzchołków w instancji nie jest parzysta, to pierwsza ścieżka zawiera jeden wierzchołek więcej), minimalizując łączną długość obu ścieżek.

#### Instancje
Rozważamy instancje `kroa200` i `krob200` z biblioteki TSPLib. Są to dwuwymiarowe instancje euklidesowe, tj. dla każdego wierzchołka podane są dwie współrzędne, a odległość pomiędzy wierzchołkami jest odległością euklidesową. Ważna uwaga: odległość jest zaokrąglana do liczby całkowitej stosując zaokrąglanie matematyczne. Proszę jednak, aby dalszy kod wykorzystywał jedynie macierz odległości, tj. aby był w pełni stosowalny do instancji Floyd, które będą zdefiniowane jedynie przez macierze odległości.

---

## Strona 2

### Zadanie 1. Heurystyki konstrukcyjne

#### Opis zadania
W ramach zadania należy:

- Zaimplementować wczytywanie instancji `kroa200` i `krob200` (w jednym z formatów, w jakim są dostępne) i obliczanie macierzy odległości.
- Zaimplementować algorytm zachłanny (greedy) inspirowany metodą najbliższego sąsiada (nearest neighbor) dla klasycznego problemu komiwojażera, dostosowując go do rozważanego problemu. Np. wybieramy losowo dwa startowe wierzchołki dla dwóch cykli i rozbudowujemy je zgodnie z zasadami dla klasycznego problemu komiwojażera aż do osiągnięcia cykli o zadanej liczbie wierzchołków. Można też np. pierwszy wierzchołek wybrać losowo, a drugi najodleglejszy od pierwszego.
- Zaimplementować algorytm zachłanny (greedy) inspirowany metodą rozbudowy cyklu (greedy cycle) dla klasycznego problemu komiwojażera, dostosowując go do rozważanego problemu.
- Zaimplementować algorytm UTCU regret heuristics (z żalem) na bazie algorytmu inspirowanego metodą rozbudowy cyklu – stosujemy 2-regret (2-żal).
- Zaimplementować algorytm z ważonym 2-żalem. Domyślnie przyjmujemy równe wagi (jedna ze zmienionym znakiem), ale można poeksperymentować z innymi parametrami.
- Opcjonalnie można zaimplementować jeszcze inną heurystykę konstrukcyjną.
- Wykonać eksperymenty obliczeniowe. Na każdej instancji każdy algorytm należy uruchomić 100 razy, wykorzystując różne wierzchołki startowe.

#### Heurystyka najbliższego sąsiada (nearest neighbor) dla problemu komiwojażera
1. Wybierz (np. losowo) wierzchołek startowy.
2. Powtarzaj:
   - Dodaj do rozwiązania wierzchołek (i prowadzącą do niego krawędź) najbliższy ostatnio dodanemu.
3. Dopóki nie zostaną dodane wszystkie wierzchołki.
4. Dodaj krawędź z ostatniego do pierwszego wierzchołka.

Istnieje też inna wersja, w której rozważa się wszystkie możliwe wierzchołki (nie tylko na końcu).

#### Metoda rozbudowy cyklu (greedy cycle)
1. Wybierz (np. losowo) wierzchołek startowy.
2. Wybierz najbliższy wierzchołek i stwórz z tych dwóch wierzchołków niepełny cykl.
3. Powtarzaj:
   - Wstaw do bieżącego cyklu w najlepsze możliwe miejsce wierzchołek powodujący najmniejszy wzrost długości cyklu.

---

## Strona 3

4. Dopóki nie zostaną dodane wszystkie wierzchołki.

### Heurystyki zachłanne oparte na żalu – regret heuristics
- Załóżmy, że element możemy wstawiać do rozwiązania w różne miejsca (np. wierzchołki w różne miejsca trasy TSP).
- Np. dwa elementy `a` i `b`:
  - Koszty wstawienia `a`: `[1, 2, 3, 4, 5]`
  - Koszty wstawienia `b`: `[5, 9, 10, 12, 15]`
- Zgodnie z regułą zachłanną wybralibyśmy `a` (1 < 5).
- Тогда jednak możemy zabłokować sobie możliwość wstawienia `b` przy koszcie 5, a zostanie nam jedynie miejsce z kosztem 9.
- `k-żal` (k-regret) to suma różnic pomiędzy najlepszym a `k-1` kolejnymi opcjami wstawienia.
- `2-żal` to różnica pomiędzy pierwszą a drugą opcją.
- Wybieramy element o największym żalu i wstawiamy go w najlepsze miejsce.
- Możemy też ważyć żal z regułą zachłanną (oceną pierwszej opcji).

### Sprawozdanie
W sprawozdaniu należy umieścić:

- Krótki opis zadania.
- Opis wszystkich zaimplementowanych algorytmów w pseudokodzie. Uwaga: pseudokod to nie jest jednozdaniowy, deklaratywny opis. Nie jest to także docelowy kod.
- Wyniki eksperymentu obliczeniowego. Dla każdej kombinacji instancja/algorytm należy podać średnią, minimalną i maksymalną wartość. Spodziewamy się, że czasy wykonania tych algorytmów powinny być pomijalne, więc nie musimy ich raportować. Sugerowany format tabeli:

|                | Instancja 1         | Instancja 2         |
|----------------|---------------------|---------------------|
| Metoda 1       | średnia (min - max) | średnia (min - max) |
| Metoda 2       | średnia (min - max) | średnia (min - max) |

- Wizualizacje najlepszych rozwiązań dla każdej kombinacji, podobne do powyższego przykładu rozwiązania.
- Wnioski.
- Kod programu (np. w postaci linku).