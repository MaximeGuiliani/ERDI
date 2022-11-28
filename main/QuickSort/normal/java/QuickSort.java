package main.QuickSort.normal.java;


import java.io.BufferedWriter;
import java.io.FileWriter;
import java.io.IOException;
import java.util.Random;

class QuickSort {

	static void swap(int[] arr, int i, int j)
	{
		int temp = arr[i];
		arr[i] = arr[j];
		arr[j] = temp;
	}


	static int partition(int[] arr, int low, int high)
	{

		int pivot = arr[high];

		int i = (low - 1);

		for (int j = low; j <= high - 1; j++) {
			if (arr[j] < pivot) {

				i++;
				swap(arr, i, j);
			}
		}
		swap(arr, i + 1, high);
		return (i + 1);
	}

	static void quickSort(int[] arr, int low, int high)
	{
		if (low < high) {
			int pi = partition(arr, low, high);
			quickSort(arr, low, pi - 1);
			quickSort(arr, pi + 1, high);
		}
	}

	static void printArray(int[] arr, int size)
	{
		for (int i = 0; i < size; i++)
			System.out.print(arr[i] + " ");

		System.out.println();
	}

	public static void main(String[] args)
	{
		Random rd = new Random();	
		int mean = 0;
		String result = "";
		int[] sizes = {2,10_000,25_000,50_000, 100_000,250_000,500_000,1_000_000,5_000_000,10_000_000} ;
		int[] arr = {2,10_000,25_000,50_000, 100_000,250_000,500_000,1_000_000,5_000_000,10_000_000} ;

		 for (int i = 0; i < 10; i++){
		
			arr = new int[sizes[i]];
			for (int j = 0; j < sizes[i]; j++) {
				arr[i] = rd.nextInt();
				System.out.println(arr[i]);
			}
			 for(int j = 0 ; j <10 ; j++){
				arr = new int[sizes[i]];
				//arr.sort();
			 }
			result +=(mean/10);
		arr[i] = rd.nextInt();
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
}

