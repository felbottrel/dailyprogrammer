import java.io.*;
import java.util.*;

public class clg243{
	private static ArrayList<Integer> find_divisors(int n){
		ArrayList<Integer> divisors = new ArrayList<>();
		for(int i = 1; i <= n; i++){
			if(n % i == 0) divisors.add(i);
		}
		return divisors;
	}

	private static void deficient(int n){
		int sum = 0;
		ArrayList<Integer> divisors = find_divisors(n);
		for(int i = 0; i < divisors.size(); i++){
			sum += divisors.get(i);
		}

		if(sum > 2*n){
			System.out.println(n + " abundant by "+ (sum - 2*n) );
		}else if(sum < 2*n){
			System.out.println(n + " deficient");
		}else{
			System.out.println(n + " perfect");
		}
	}

	public static void main(String[] args){
		deficient(6);
	}
}