# Simulacija dvostrukog klatna
**Ciljna ocena: 10**

## Opis problema
Dvostruko klatno predstavlja haotični sistem koji se sastoji od dva povezana klatna. 
Prvi segment klatna je okačen za fiksnu tačku, dok je drugi segment okačen za kraj prvog segmenta.
Sistem pokazuje veoma kompleksno, nepredvidivo ponašanje uprkos jednostavnoj strukturi.

Problem se rešava numeričkim integrisanjem sistema od 4 diferencijalne jednačine prvog reda:
- θ₁, θ₂ - uglovi segmenata
- ω₁, ω₂ - ugaone brzine segmenata

## Metode rešavanja

### Numerička integracija
- Koristi se Runge-Kutta metoda 4. reda za numeričku integraciju
- Sistem diferencijalnih jednačina se rešava iterativno kroz vremenske korake

### Paralelizacija
**Python implementacija:**
- Sekvencijalna verzija - direktna implementacija RK4 metode
- Paralelna verzija - koristi multiprocessing za simulaciju različitih početnih uslova

**Rust implementacija:**
- Sekvencijalna verzija - optimizovana implementacija
- Paralelna verzija - koristi thread-ove za podelu vremenskih koraka ili početnih uslova

## Eksperimenti skaliranja

### Jako skaliranje
**Problem fiksne veličine:** Simulacija jednog dvostrukog klatna sa fiksnim brojem vremenskih koraka (npr. 100,000)

**Paralelizacija:**
- Možemo podeliti vremenske korake između procesa/niti
- Svaki proces/nit računa deo trajektorije
- Na kraju kombinujemo rezultate

**Merenja:** Testiranje sa 1,2,4,8,16 niti na istom problemu

### Slabo skaliranje
**Problem skalabilne veličine:** Konstantan posao po procesu/niti

**Implementacija:**
- 1 proces = 50,000 vremenskih koraka
- 2 procesa = 100,000 ukupno koraka
- 4 procesa = 200,000 ukupno koraka

**Cilj:** Vreme izvršavanja treba da ostane konstantno jer se posao po procesu ne menja

## Detalji paralelizacije

### Python paralelizacija
**Strategija - Podela vremenskih koraka:**
- Master proces deli simulaciju na segmente
- Svaki worker proces simulira deo trajektorije
- Rezultati se spajaju hronološki

### Rust paralelizacija
**Strategija 1 - Rayon crate:**
- Paralelizacija kroz `.par_iter()` za vremenske korake
- Thread pool automatski upravlja radnim nitima

**Strategija 2 - Manuel threading:**
- Eksplicitna podela posla između niti
- Mutex/Arc za deljenje rezultata
- Join na kraju za kombinovanje

**Izazovi paralelizacije:**
- Dvostruko klatno je sekvencijalan problem (svaki korak zavisi od prethodnog)
- Rešenje: podela na veće blokove vremenskih koraka ili simulacija više različitih scenarija

## Vizuelizacija

### Tehnička implementacija (Rust)
**Biblioteka:** Plotters crate za generisanje grafika i animacija

**Komponente vizuelizacije:**

1. **Real-time animacija:**
   - Crtanje 2 segmenta klatna kao linije
   - Prikaz trenutnih pozicija
   - 60 FPS
2. **Trajektorija:**
   - Plot putanje druge mase kroz prostor
   - Različite boje za različite delove trajektorije
   - Po mogućnosti fade-out efekat za starije pozicije
3. **Energija sistema:**
   - Plot kinetičke, potencijalne i ukupne energije kroz vreme
   - Verifikacija konzervacije energije

**Izlazni formati:**
- PNG sekvence
- GIF animacija
- interaktivni HTML
- itd.

## Merenje performansi
- 30 pokretanja po konfiguraciji za statički značajne rezultate
- Računanje speedup faktora i efikasnosti
- Poređenje sa teoretskim maksimumima

---
## Dodatak za diplomski rad

### Implementacija u Go-u

Pored Python i Rust implementacija, razvijena je i implementacija u jeziku GoLang (Go).
Golang se uvodi jer je jednostavan i čitljiv jezik sa minimalnom sintaksom. Takođe, sadrži prirodnu podršku za konkurentnost kroz *goroutines* i *channels*.
Pruža balans između brzine izvršavanja i jednostavnosti razvoja.

Za sekvencijalnu verziju bi se koristila direktna implementacija Runge-Kutta metode, slično kao kod Python i Rust implementacije.
Za paralelnu verziju koristio bih *goroutines* za podelu posla.
Rezultate bih objedinio preko kanala.

Na samom kraju, uporedio bih implementacije ova 3 jezika u vidu složenosti koda, brzine razvoja, brzine izvršavanja, paralelizacija itd.

### Opciono: Trostruko klatno (Triple Pendulum)

Proširenje dvostrukog klatna gde se treći segment dodaje na kraj drugog.
Sistem postaje još haotičniji, a i broj stanja raste (potrebno je rešavati *6 diferencijalnih jednačina prvog reda* (θ₁, θ₂, θ₃, ω₁, ω₂, ω₃)).

Implementacija bi bila ostvarena u jednom od prethodna 3 jezika, u zavisnosti od performansi. Ovaj problem je dosta izazovniji za paralelizaciju, ali bi vizuelizacija bila još atraktivnija.
