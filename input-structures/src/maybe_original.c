int Smooth_Array_Zhmakin_Fursenko(double *Fs, int Nmax, double smooth_intensity, double *Fi, double * Ftd)
{
	int n;
	double sign, f1, f2; // double_smooth_intensity 
	for (n = 0; n <= Nmax - 1; n++)
		Fi[n] = smooth_intensity*(Fs[n + 1] - Fs[n]);
	for (n = 1; n <= Nmax - 1; n++)
		Ftd[n] = Fs[n] + (Fi[n] - Fi[n - 1]);
	Ftd[0] = Fs[0];
	Ftd[Nmax] = Fs[Nmax];
	for (n = 0; n <= Nmax - 1; n++)
		Fs[n] = Ftd[n + 1] - Ftd[n];
	for (n = 0; n <= Nmax - 1; n++)
	{
		Fi[n] = smooth_intensity*(Ftd[n + 1] - Ftd[n]);
		sign = -1;
		if (Fi[n] >= 0)
			sign = 1;
		Fi[n] = fabs(Fi[n]);
		if (n == 0)
		{
			f2 = sign*Fs[1];
			if (f2<Fi[0])
				Fi[0] = f2;
		}
		else if (n == Nmax - 1)
		{
			f1 = sign*Fs[Nmax - 2];
			if (f1<Fi[Nmax - 1])
				Fi[Nmax - 1] = f1;
		}
		else
		{
			f1 = sign*Fs[n - 1];
			f2 = sign*Fs[n + 1];
			if (f1<Fi[n])
				Fi[n] = f1;
			if (f2<Fi[n])
				Fi[n] = f2;
		}
		if (Fi[n]<0)
			Fi[n] = 0;
		else
			Fi[n] = sign*Fi[n];
	}
	for (n = 1; n <= Nmax - 1; n++)
		Fs[n] = Ftd[n] - (Fi[n] - Fi[n - 1]);
	Fs[0] = Ftd[0];
	return 0;
}

