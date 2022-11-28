



import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Random;
import java.util.concurrent.ExecutorService;
import java.util.concurrent.Executors;
import java.util.concurrent.atomic.AtomicInteger;

public class Qs {
    public static void main(String args[]) {
        // unsorted integer array   
        int[] sizes = {2,10_000,25_000,50_000, 100_000,250_000,500_000,1_000_000,5_000_000,10_000_000} ;
        int[] unsorted = new int[2];
        Random rd = new Random();
        String result = "";

        // for all array size
        for (int i = 0; i < 10; i++){
            long mean = 0;
			unsorted = new int[sizes[i]];
            // mean for a fixed size
            for(int j = 0 ; j <10 ; j++){
                 // random array fill
                for (int k = 0; k < sizes[i]; k++) {
                    unsorted[i] = rd.nextInt();
                }
                long start = System.currentTimeMillis();
                trierRapidement(unsorted, 0, sizes[i] - 1);
                long end = System.currentTimeMillis();

                mean =mean + end - start;
                System.out.println(mean);

			 }
			result +=sizes[i] + ", " +(mean/10) + "\n";
	}



	String fileName = "qs_Java.csv";
    try {
    BufferedWriter writer = new BufferedWriter(new FileWriter(fileName));
        writer.write(result);
        writer.close();

    } catch (IOException e) {
        e.printStackTrace();
    }

    }

    static final int taille = Integer.MAX_VALUE / 20;   // Longueur du tableau à trier
    static final int[] tableau = new int[taille];       // Le tableau d'entiers à trier 
    static final int borne = 10 * taille;               // Valeur maximale dans le tableau

    private static void echangerElements(int[] t, int m, int n) {
        int temp = t[m];
        t[m] = t[n];
        t[n] = temp;
    }

    public static int partitionner(int[] t, int début, int fin) {
        int v = t[fin] ;                               // Choix (arbitraire) du pivot : t[fin]
        int place = début ;                            // Place du pivot, à droite des éléments déplacés
        for (int i = début ; i<fin ; i++) {            // Parcours du *reste* du tableau
            if (t[i] < v) {                            // Cette valeur t[i] doit être à droite du pivot
                echangerElements(t, i, place) ;        // On le place à sa place
                place++ ;                              // On met à jour la place du pivot
            }
        }
        echangerElements(t, place, fin) ;              // Placement définitif du pivot
        return place ;
    }

    static void trierRapidement(int[] t, int début, int fin) {
        if (début < fin) {                             // S'il y a un seul élément, il n'y a rien à faire!
            int p = partitionner(t, début, fin) ;      
            trierRapidement(t, début, p-1) ;
            trierRapidement(t, p+1, fin) ;
        }
    }



    private static void trierRapidement2(int[] t, int début, int fin) {
        if (début < fin) {                             // S'il y a un seul élément, il n'y a rien à faire!
            int p = partitionner2(t, début, fin);
            trierRapidement2(t, début, p - 1);
            trierRapidement2(t, p + 1, fin);
        }

        else if (début == fin && counter.incrementAndGet() >= taille) {
            synchronized (Qs.class) {
                Qs.class.notifyAll();
            }
        }

    }

    private static int partitionner2(int[] t, int début, int fin) {
        if (counter.incrementAndGet() >= taille) {
            synchronized (Qs.class) {
                Qs.class.notifyAll();
            }
        }
        return partitionner(t,  début, fin);
    }


    static final int nbThread = 4;
    final static ExecutorService executeur = Executors.newFixedThreadPool(nbThread);
    static final AtomicInteger counter = new AtomicInteger();

    private static class SortingTask implements Runnable {

        final int début, fin;

        SortingTask(int début, int fin) {
            this.début = début;
            this.fin = fin;
        }

        @Override
        public void run() {
            if (début > fin)
                return;
            int p = partitionner2(tableau, début, fin);

            if (fin - (p + 1)<= taille / 100) {
                trierRapidement2(tableau, début, p - 1);
                trierRapidement2(tableau, p + 1, fin);
                return;
            }

            executeur.execute(new SortingTask(début, p - 1));
            executeur.execute(new SortingTask(p + 1, fin));
            return;
        }

    }
    // vérifie si un tableau est bien trié
    static boolean isSorted(int[] array) {
        for (int i = 0; i < array.length - 1; i++) {
            if (array[i] > array[i + 1])
                return false;
        }
        return true;
    }

    public static void sort() throws InterruptedException {
        if (tableau.length <= 1)
            return;

        executeur.execute(new SortingTask(0, taille - 1));
        synchronized (Qs.class) {
            while (counter.get() < taille) {
                Qs.class.wait();
            }
        }
    }
}